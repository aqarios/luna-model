#include <cstdint>
#include <iostream>
#include <vector>

#include <luna_model.hpp>

int main() {
    std::cout << "LunaModel " << lm::version() << "\n";

    lm::Model model;
    std::vector<std::uint8_t> bytes = model.serialize();
    std::cout << "serialized model: " << bytes.size() << " bytes\n";

    lm::Model restored = lm::Model::deserialize(bytes);
    std::cout << "deserialized; re-serialized: " << restored.serialize().size() << " bytes\n";

    lm::Model cloned = restored.clone();
    std::cout << "cloned model ok\n";

    return 0;
}
