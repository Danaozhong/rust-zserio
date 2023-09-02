package reference_modules.ambiguous_types.other;

// This type "clones" the type in main.zs.
// Since this file is imported from main.zs,
// the type is in the evaluation scope, but the
// compiler should pick the definition in main.zs,
// because local definitions are more relevant than
// imported ones.
subtype bool CustomType;