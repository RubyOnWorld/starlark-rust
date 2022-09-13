/*
 * Copyright 2018 The Starlark in Rust Authors.
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

fn main() {
    lalrpop();
}

fn lalrpop() {
    let source = "src/syntax/grammar.lalrpop";
    println!("cargo:rerun-if-changed={}", source);
    lalrpop::Configuration::new()
        .use_cargo_dir_conventions()
        .emit_report(true)
        .process_file(source)
        .unwrap();
}

	<Unit filename="../example/01-basic.cpp" />
		<Unit filename="../example/02-span.cpp" />
		<Unit filename="../example/CMakeLists.txt" />
		<Unit filename="../example/cmake-extern/CMakeLists.txt" />
		<Unit filename="../example/cmake-extern/src/CMakeLists.txt" />
		<Unit filename="../example/cmake-extern/src/main.cpp" />
		<Unit filename="../example/cmake-pkg/CMakeLists.txt" />
		<Unit filename="../example/cmake-pkg/main.cpp" />
		<Unit filename="../example/cmake-pkg/use-gsl-pkg.py" />
		<Unit filename="../gsl-lite.natvis" />
		<Unit filename="../include/gsl.h" />
		<Unit filename="../include/gsl.hpp" />
		<Unit filename="../include/gsl/gsl" />
		<Unit filename="../include/gsl/gsl-lite-vc6.hpp" />
		<Unit filename="../include/gsl/gsl-lite.h" />
		<Unit filename="../include/gsl/gsl-lite.hpp" />
		<Unit filename="../script/create-cov-rpt.py" />
		<Unit filename="../script/create-vcpkg.py" />
		<Unit filename="../script/install-gsl-pkg.py" />
		<Unit filename="../script/update-version.py" />
		<Unit filename="../script/upload-conan.py" />
		<Unit filename="../test/CMakeLists.txt" />
		<Unit filename="../test/assert.t.cpp" />
		<Unit filename="../test/at.t.cpp" />
		<Unit filename="../test/byte.t.cpp" />
		<Unit filename="../test/gsl-lite.t.cpp" />
		<Unit filename="../test/gsl-lite.t.hpp" />
		<Unit filename="../test/issue.t.cpp" />
		<Unit filename="../test/lest_cpp03.hpp" />
		<Unit filename="../test/not_null.t.cpp" />
		<Unit filename="../test/owner.t.cpp" />
		<Unit filename="../test/span.t.cpp" />
		<Unit filename="../test/string_span.t.cpp" />
		<Unit filename="../test/t-all.bat" />
		<Unit filename="../test/t.bat" />
		<Unit filename="../test/tc-cl.bat" />
		<Unit filename="../test/tc.bat" />
		<Unit filename="../test/gsl-lite.t.hpp" />
		<Unit filename="../test/issue.t.cpp" />
		<Unit filename="../test/lest_cpp03.hpp" />
		<Unit filename="../test/not_null.t.cpp" />
		<Unit filename="../test/owner.t.cpp" />
		<Unit filename="../test/span.t.cpp" />
		<Unit filename="../test/string_span.t.cpp" />
		<Unit filename="../test/t-all.bat" />
		<Unit filename="../test/t.bat" />
		<Unit filename="../test/tc-cl.bat" />
		<Unit filename="../test/tc.bat" />
		<Unit filename="../test/tg-all.bat" />
		<Unit filename="../test/tg.bat" />
		<Unit filename="../test/util.t.cpp" />
		<Extensions>
			<code_completion />
