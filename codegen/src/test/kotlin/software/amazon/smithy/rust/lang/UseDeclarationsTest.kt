package software.amazon.smithy.rust.lang

import io.kotest.matchers.shouldBe
import org.junit.jupiter.api.Test
import software.amazon.smithy.rust.codegen.lang.UseDeclarations
import software.amazon.smithy.rust.testutil.shouldCompile

class UseDeclarationsTest {
    private fun useDecl() = UseDeclarations("lib.rs", "test")

    @Test
    fun `it produces valid use decls`() {
        val sut = useDecl()
        sut.addImport("std::collections", "HashSet")
        sut.addImport("std::borrow", "Cow")
        sut.toString() shouldBe "use std::borrow::Cow;\nuse std::collections::HashSet;"
        sut.toString().shouldCompile()
    }

    @Test
    fun `it deduplicates use decls`() {
        val sut = useDecl()
        sut.addImport("std::collections", "HashSet")
        sut.addImport("std::collections", "HashSet")
        sut.addImport("std::collections", "HashSet")
        sut.toString() shouldBe "use std::collections::HashSet;"
        sut.toString().shouldCompile()
    }

    @Test
    fun `it supports aliasing`() {
        val sut = useDecl()
        sut.addImport("std::collections", "HashSet", "HSet")
        sut.addImport("std::collections", "HashSet")
        sut.toString() shouldBe "use std::collections::HashSet as HSet;\nuse std::collections::HashSet;"
        sut.toString().shouldCompile()
    }
}
