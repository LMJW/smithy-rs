/*
 * Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License").
 * You may not use this file except in compliance with the License.
 * A copy of the License is located at
 *
 *  http://aws.amazon.com/apache2.0
 *
 * or in the "license" file accompanying this file. This file is distributed
 * on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
 * express or implied. See the License for the specific language governing
 * permissions and limitations under the License.
 *
 *
 */

package software.amazon.smithy.rust.codegen.lang

import software.amazon.smithy.codegen.core.SymbolDependency
import software.amazon.smithy.codegen.core.SymbolDependencyContainer
import software.amazon.smithy.rust.codegen.smithy.RuntimeConfig

sealed class DependencyLocation
data class CratesIo(val version: String) : DependencyLocation()
data class Local(val path: String? = null) : DependencyLocation()

data class RustDependency(
    val name: String,
    val location: DependencyLocation
) : SymbolDependencyContainer {
    override fun getDependencies(): List<SymbolDependency> {
        return listOf(
            SymbolDependency.builder().packageName(name).version(this.version()).putProperty(PropKey, this).build()
        )
    }

    private fun version(): String = when (location) {
        is CratesIo -> location.version
        is Local -> "local"
    }

    override fun toString(): String {
        return when (location) {
            is CratesIo -> """$name = "${location.version}""""
            is Local -> """$name = { path = "${location.path}/$name" }"""
        }
    }

    companion object {
        val Http: RustDependency = RustDependency("http", CratesIo("0.2"))
        fun SmithyTypes(runtimeConfig: RuntimeConfig) =
            RustDependency("${runtimeConfig.cratePrefix}-types", Local(runtimeConfig.relativePath))

        fun SmithyHttp(runtimeConfig: RuntimeConfig) = RustDependency(
            "${runtimeConfig.cratePrefix}-http", Local(runtimeConfig.relativePath)
        )

        private val PropKey = "rustdep"

        fun fromSymbolDependency(symbolDependency: SymbolDependency) =
            symbolDependency.getProperty(PropKey, RustDependency::class.java).get()
    }
}
