initSidebarItems({"enum":[["StaticFields","Fields for a static method"],["SubstructureFields","A summary of the possible sets of fields."]],"fn":[["combine_substructure",""],["cs_and","cs_binop with binop == and"],["cs_binop","Use a given binop to combine the result of calling the derived method on all the fields."],["cs_fold","Fold the fields. `use_foldl` controls whether this is done left-to-right (`true`) or right-to-left (`false`)."],["cs_or","cs_binop with binop == or"],["cs_same_method","Call the method that is being derived on all the fields, and then process the collected results. i.e."],["cs_same_method_fold","Fold together the results of calling the derived method on all the fields. `use_foldl` controls whether this is done left-to-right (`true`) or right-to-left (`false`)."]],"mod":[["ty","A mini version of ast::Ty, which is easier to use, and features an explicit `Self` type to use when specifying impls to be derived."]],"struct":[["FieldInfo","Summary of the relevant parts of a struct/enum field."],["MethodDef",""],["Substructure","All the data about the data structure/method being derived upon."],["TraitDef",""]],"type":[["CombineSubstructureFunc","Combine the values of all the fields together. The last argument is all the fields of all the structures."],["EnumNonMatchCollapsedFunc","Deal with non-matching enum variants.  The tuple is a list of identifiers (one for each `Self` argument, which could be any of the variants since they have been collapsed together) and the identifiers holding the variant index value for each of the `Self` arguments.  The last argument is all the non-`Self` args of the method being derived."]]});