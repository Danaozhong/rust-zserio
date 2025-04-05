# Use this to generate the zserio bindings for python and C++ using the zserio reference implementation
zserio reference_modules/all.zs \
    -setTopLevelPackage pygen \
    -python ./compare-ref-impl-tests/python \
    -cpp ./compare-ref-impl-tests/cppgen
