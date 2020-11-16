/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

package software.amazon.smithy.rust.codegen.smithy

import software.amazon.smithy.build.PluginContext
import software.amazon.smithy.codegen.core.SymbolProvider
import software.amazon.smithy.codegen.core.writer.CodegenWriterDelegator
import software.amazon.smithy.model.Model
import software.amazon.smithy.model.neighbor.Walker
import software.amazon.smithy.model.shapes.ServiceShape
import software.amazon.smithy.model.shapes.Shape
import software.amazon.smithy.model.shapes.ShapeVisitor
import software.amazon.smithy.model.shapes.StringShape
import software.amazon.smithy.model.shapes.StructureShape
import software.amazon.smithy.model.shapes.UnionShape
import software.amazon.smithy.model.traits.EnumTrait
import software.amazon.smithy.rust.codegen.lang.RustDependency
import software.amazon.smithy.rust.codegen.lang.RustWriter
import software.amazon.smithy.rust.codegen.smithy.generators.CargoTomlGenerator
import software.amazon.smithy.rust.codegen.smithy.generators.EnumGenerator
import software.amazon.smithy.rust.codegen.smithy.generators.HttpProtocolGenerator
import software.amazon.smithy.rust.codegen.smithy.generators.LibRsGenerator
import software.amazon.smithy.rust.codegen.smithy.generators.ProtocolConfig
import software.amazon.smithy.rust.codegen.smithy.generators.ProtocolGeneratorFactory
import software.amazon.smithy.rust.codegen.smithy.generators.ServiceGenerator
import software.amazon.smithy.rust.codegen.smithy.generators.StructureGenerator
import software.amazon.smithy.rust.codegen.smithy.generators.UnionGenerator
import software.amazon.smithy.rust.codegen.smithy.protocols.ProtocolLoader
import software.amazon.smithy.rust.codegen.smithy.transformers.OperationNormalizer
import software.amazon.smithy.rust.codegen.util.CommandFailed
import software.amazon.smithy.rust.codegen.util.runCommand
import java.util.logging.Logger

private val PublicModules = listOf("error", "operation", "model")

class CodegenVisitor(context: PluginContext) : ShapeVisitor.Default<Unit>() {

    private val logger = Logger.getLogger(javaClass.name)
    private val settings = RustSettings.from(context.model, context.settings)

    private val symbolProvider: SymbolProvider
    private val writers: CodegenWriterDelegator<RustWriter>
    private val fileManifest = context.fileManifest
    private val model: Model
    private val protocolConfig: ProtocolConfig
    private val protocolGenerator: ProtocolGeneratorFactory<HttpProtocolGenerator>
    private val httpGenerator: HttpProtocolGenerator

    init {
        val symbolVisitorConfig = SymbolVisitorConfig(runtimeConfig = settings.runtimeConfig)
        val bootstrapProvider = RustCodegenPlugin.BaseSymbolProvider(context.model, symbolVisitorConfig)
        model = OperationNormalizer(bootstrapProvider).addOperationInputs(context.model)
        symbolProvider =
            RustCodegenPlugin.BaseSymbolProvider(model, SymbolVisitorConfig(runtimeConfig = settings.runtimeConfig))
        val service = settings.getService(context.model)
        val (protocol, generator) = ProtocolLoader.Default.protocolFor(context.model, service)
        protocolGenerator = generator

        protocolConfig = ProtocolConfig(model, symbolProvider, settings.runtimeConfig, service, protocol)
        writers = CodegenWriterDelegator(
            context.fileManifest,
            // TODO: load symbol visitor from integrations; 2d
            symbolProvider,
            RustWriter.Factory
        )
        httpGenerator = protocolGenerator.buildProtocolGenerator(protocolConfig)
    }

    fun execute() {
        logger.info("generating Rust client...")
        val service = settings.getService(model)
        val serviceShapes = Walker(model).walkShapes(service)
        serviceShapes.forEach { it.accept(this) }
        writers.useFileWriter("Cargo.toml") {
            val cargoToml = CargoTomlGenerator(
                settings,
                it,
                writers.dependencies.map { dep -> RustDependency.fromSymbolDependency(dep) }.distinct()
            )
            cargoToml.render()
        }
        writers.useFileWriter("src/lib.rs", "crate::lib") {
            // TODO: a more structured method of signaling what modules should get loaded.
            val modules = PublicModules.filter { writers.writers.containsKey("src/$it.rs") }
            LibRsGenerator(modules, it).render()
        }
        writers.flushWriters()
        try {
            "cargo fmt".runCommand(fileManifest.baseDir)
        } catch (_: CommandFailed) {
            logger.warning("Generated output did not parse")
        }
    }

    override fun getDefault(shape: Shape?) {
    }

    override fun structureShape(shape: StructureShape) {
        // super.structureShape(shape)
        logger.info("generating a structure...")
        writers.useShapeWriter(shape) {
            StructureGenerator(model, symbolProvider, it, shape).render()
        }
    }

    override fun stringShape(shape: StringShape) {
        shape.getTrait(EnumTrait::class.java).map { enum ->
            writers.useShapeWriter(shape) { writer ->
                EnumGenerator(symbolProvider, writer, shape, enum).render()
            }
        }
    }

    override fun unionShape(shape: UnionShape) {
        writers.useShapeWriter(shape) {
            UnionGenerator(model, symbolProvider, it, shape).render()
        }
    }

    override fun serviceShape(shape: ServiceShape) {
        ServiceGenerator(writers, httpGenerator, protocolConfig).render()
    }
}
