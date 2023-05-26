window.SIDEBAR_ITEMS = {"enum":[["ResolveError",""]],"fn":[["check_for_cycles","Check for dependency cycles by doing a Depth First Search of the graph, starting at the root."],["check_for_duplicate_versions","As a workaround for the lack of “proper” dependency merging, we’ll make sure only one copy of each package is in the dependency tree. If the same package is included in the tree multiple times, they all need to use the exact same version otherwise it’s an error."],["log_dependencies",""],["print_cycle",""],["resolve","Given the [`PackageInfo`] for a root package, resolve its dependency graph and figure out how it could be executed."],["resolve_dependency_graph",""],["resolve_filesystem_mapping",""],["resolve_package","Given a [`DependencyGraph`], figure out how the resulting “package” would look when loaded at runtime."]]};