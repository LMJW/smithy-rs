package software.amazon.smithy.rust.codegen.smithy

import java.util.logging.Logger
import software.amazon.smithy.build.PluginContext
import software.amazon.smithy.model.neighbor.Walker
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
import software.amazon.smithy.rust.codegen.smithy.generators.LibRsGenerator
import software.amazon.smithy.rust.codegen.smithy.generators.StructureGenerator
import software.amazon.smithy.rust.codegen.smithy.generators.UnionGenerator
import software.amazon.smithy.rust.codegen.util.runCommand
import software.amazon.smithy.vended.CodegenWriterDelegator

class CodegenVisitor(private val context: PluginContext) : ShapeVisitor.Default<Unit>() {

    private val logger = Logger.getLogger(javaClass.name)
    private val settings = RustSettings.from(context.model, context.settings)
    private val symbolProvider = SymbolVisitor(context.model, config = SymbolVisitorConfig(runtimeConfig = settings.runtimeConfig))
    private val writers = CodegenWriterDelegator(
        context.fileManifest,
        // TODO: load symbol visitor from integrations
        symbolProvider,
        RustWriter.Factory
    )

    fun execute() {
        logger.info("generating Rust client...")
        val modelWithoutTraits = context.modelWithoutTraitShapes
        val service = settings.getService(context.model)
        val serviceShapes = Walker(modelWithoutTraits).walkShapes(service)
        serviceShapes.forEach { it.accept(this) }
        writers.useFileWriter("Cargo.toml") {
            val cargoToml = CargoTomlGenerator(settings, it, writers.dependencies.map { dep -> RustDependency.fromSymbolDependency(dep) }.distinct())
            cargoToml.render()
        }
        writers.useFileWriter("src/lib.rs") {
            val modules = mutableListOf<String>()
            if (writers.writers.containsKey("src/model.rs")) {
                modules.add("model")
            }
            if (writers.writers.containsKey("src/error.rs")) {
                modules.add("error")
            }
            LibRsGenerator(modules, it).render()
        }
        writers.flushWriters()
        "cargo fmt".runCommand(context.fileManifest.baseDir)
    }

    override fun getDefault(shape: Shape?) {
    }

    override fun structureShape(shape: StructureShape) {
        // super.structureShape(shape)
        logger.info("generating a structure...")
        writers.useShapeWriter(shape) {
            StructureGenerator(context.model, symbolProvider, it, shape).render()
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
            UnionGenerator(context.model, symbolProvider, it, shape).render()
        }
    }
}
