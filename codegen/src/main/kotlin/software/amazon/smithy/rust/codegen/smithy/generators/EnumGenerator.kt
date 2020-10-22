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

package software.amazon.smithy.rust.codegen.smithy.generators

import software.amazon.smithy.codegen.core.SymbolProvider
import software.amazon.smithy.model.shapes.StringShape
import software.amazon.smithy.model.traits.EnumDefinition
import software.amazon.smithy.model.traits.EnumTrait
import software.amazon.smithy.rust.codegen.lang.RustWriter
import software.amazon.smithy.rust.codegen.lang.rustBlock
import software.amazon.smithy.rust.codegen.lang.withBlock
import software.amazon.smithy.rust.codegen.smithy.RuntimeType

class EnumGenerator(
    symbolProvider: SymbolProvider,
    private val writer: RustWriter,
    shape: StringShape,
    enumTrait: EnumTrait
) {

    fun render() {
        // pub enum Blah { V1, V2, .. }
        renderEnum()
        writer.insertTrailingNewline()
        // impl From<str> for Blah { ... }
        renderFromStr()
        writer.insertTrailingNewline()
        // impl Blah { pub fn as_str(&self) -> &str
        renderAsStr()
    }

    private fun EnumDefinition.derivedName(): String {
        // TODO: For unnamed enums, generate a newtype that wraps the string. We can provide a &[&str] with the valid
        // values.

        // Because enum variants always start with an upper case letter, they will never
        // conflict with reserved words (which are always lower case), therefore, we never need
        // to fall back to raw identifiers
        return deriveName(name.orElse(null), value)
    }

    private val sortedMembers: List<EnumDefinition> = enumTrait.values.sortedBy { it.value }
    private val enumName = symbolProvider.toSymbol(shape).name
    private fun renderEnum() {
        writer.write("#[non_exhaustive]")
        // Enums can only be strings, so we can derive Eq
        writer.write("#[derive(Debug, PartialEq, Eq, Clone)]")
        writer.rustBlock("pub enum $enumName") {
            sortedMembers.forEach { member ->
                member.documentation.map { setNewlinePrefix("/// ").write(it).setNewlinePrefix("") }
                // use the name, or escape the value
                write("${member.derivedName()},")
            }
            write("Unknown(String)")
        }
    }

    private fun renderAsStr() {
        // TODO: should enums also implement AsRef<str>?
        writer.rustBlock("impl $enumName") {
            writer.rustBlock("pub fn as_str(&self) -> &str") {
                writer.rustBlock("match self") {
                    sortedMembers.forEach { member ->
                        write("""$enumName::${member.derivedName()} => "${member.value}",""")
                    }
                    write("$enumName::Unknown(s) => s.as_ref()")
                }
            }
        }
    }

    private fun renderFromStr() {
        writer.rustBlock("impl <T> \$T<T> for $enumName where T: \$T<str>", RuntimeType.From, RuntimeType.AsRef) {
            writer.withBlock("fn from(s: T) -> Self {", "}") {
                writer.withBlock("match s.as_ref() {", "}") {
                    sortedMembers.forEach { member ->
                        write(""""${member.value}" => $enumName::${member.derivedName()},""")
                    }
                    write("other => $enumName::Unknown(other.to_owned())")
                }
            }
        }
    }

    companion object {
        fun deriveName(name: String?, value: String): String {
            return name?.toPascalCase() ?: value.replace(" ", "_").toPascalCase().filter { it.isLetterOrDigit() }
        }
    }
}
