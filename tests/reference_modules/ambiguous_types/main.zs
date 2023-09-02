package reference_modules.ambiguous_types.main;

import reference_modules.ambiguous_types.other.*;

// See zserio language overview http://zserio.org/doc/ZserioLanguageOverview.html:
// Import declarations only have any effect when there is a reference to 
// a type or symbol not defined in the current package. If package map defines 
// its own Coordinate type, any reference to that within package map will be 
// resolved to the local type map.Coordinate, even when one or more of the 
// imported packages also define a type named Coordinate.
//
subtype int32 CustomType;

struct AmbiguousTypesStruct {
    // should resolve to reference_modules.ambiguous_types.main.CustomType,
    // not reference_modules.ambiguous_types.other.CustomType.
    CustomType value;
};
