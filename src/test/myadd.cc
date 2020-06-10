#include <tvm/runtime/packed_func.h>
#include <tvm/runtime/registry.h>

namespace tvm{
namespace test{
    using tvm::runtime::TVMArgs;
    using tvm::runtime::TVMRetValue;
    void MyAdd(TVMArgs args, TVMRetValue* rv) {
    // automatically convert arguments to desired type.
    int a = args[0];
    int b = args[1];
    std::cout << "excellent!!!" << std::endl;
    // automatically assign value return to rv
    *rv = a + b;
}

/* void CallMyAdd(TVMArgs args, TVMRetValue* rv) {
    using tvm::runtime::PackedFunc;
    PackedFunc myadd = PackedFunc(MyAdd);
    // get back 3
    int a = args[0];
    int c = myadd(1, 2);
    std::cout << c << std::endl;
    *rv = a;
} */

/* void CallMyAdd() {
    tvm::runtime::PackedFunc myadd = tvm::runtime::PackedFunc(MyAdd);
    // get back 3
    int c = myadd(1, 2);
} */

// register a global packed function in c++
TVM_REGISTER_GLOBAL("myadd").set_body(MyAdd);

/* TVM_REGISTER_GLOBAL("callhello")
.set_body([](TVMArgs args, TVMRetValue* rv) {
    tvm::runtime::PackedFunc f = args[0];
    f("hello world!");
}); */

} // namespace test
} // namespace tvm
