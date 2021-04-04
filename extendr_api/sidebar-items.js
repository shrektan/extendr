initSidebarItems({"attr":[["extendr",""]],"constant":[["FALSE","FALSE value eg. `r!(FALSE)`"],["NA_INTEGER","NA value for integers eg. `r!(NA_INTEGER)`"],["NA_LOGICAL","NA value for logical. `r!(NA_LOGICAL)`"],["NA_REAL","NA value for real values eg. `r!(NA_REAL)`"],["NA_STRING","NA value for strings. `r!(NA_STRING)`"],["NULL","NULL value eg. `r!(NULL)`"],["TRUE","TRUE value eg. `r!(TRUE)`"]],"enum":[["RType","Type of R objects used by [Robj::rtype]."]],"macro":[["R","Call inline R source code from Rust."],["call","The call! macro calls an R function with Rust parameters. Equivalent to `lang!(sym, params).eval()` This returns a Rust Result."],["data_frame","Create a dataframe."],["extendr_module","Define a module and export symbols to R Example:"],["factor","Create a factor."],["global","Get a global variable."],["lang","A macro for constructing R langage objects."],["list","Create a list."],["pairlist","Create a Pairlist R object from a list of name-value pairs."],["r","Convert a rust expression to an R object."],["reprint","Print via the R error stream."],["reprintln","Print with a newline via the R output stream."],["rprint","Print via the R output stream."],["rprintln","Print with a newline via the R output stream."],["sym","The sym! macro install symbols. You should cache your symbols in variables as generating them is costly."],["test","Macro for running tests."],["var","Get a local variable from the calling function or a global variable if no such variable exists."]],"mod":[["error","Error handling in Rust called from R."],["functions",""],["iter",""],["lang_macros","Argument parsing and checking."],["logical",""],["metadata","Module metadata"],["ownership","Maintain ownership of R objects."],["prelude","Common exports for extendr-api.This allows us to be more selective about exports and avoid users using deprecated features."],["rmacros","rmacros - a set of macros to call actual R functions in a rusty way."],["robj","R object handling."],["robj_ndarray",""],["thread_safety","Provide limited protection for multithreaded access to the R API."],["wrapper","Wrappers are lightweight proxies for references to R datatypes. They do not contain an Robj (see array.rs for an example of this)."]],"trait":[["Deref","Used for immutable dereferencing operations, like `*v`."],["IsNA","Return true if this primitive is NA."],["TryFrom","Simple and safe type conversions that may fail in a controlled way under some circumstances. It is the reciprocal of [`TryInto`]."],["TryInto","An attempted conversion that consumes `self`, which may or may not be expensive."]]});