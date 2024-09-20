package reference_modules.parameter_passing.function_parameter;

struct ParametrizedChild(bit:5 numIndexBits) { };

struct Parent {
    ParametrizedChild(numVertices()) children[6];

  function varsize numVertices()
  {
    return 1;
  }
}
