package reference_modules.template_instantiation.template_instantiation;

struct TemplatedStruct<T, V> {
    T value1;
    V value2;
};

// Instantiate using "instantiate"
instantiate TemplatedStruct<int32, string> InstantiatedStruct1;

// Instantiate using "subtype"
subtype TemplatedStruct<string, int16> InstantiatedStruct2;
