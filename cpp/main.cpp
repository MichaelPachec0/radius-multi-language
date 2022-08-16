#include <iostream>

int main() {
    const double PI = 3.14159251;
    double radius, tmp = 0;
    std::cout << "Enter the radius of the sphere: ";
    std::cin >> radius;

    double volume = 4 * PI * (radius * radius * radius)/3;
    std::cout << "The volume of a sphere with the radius of " << radius << " inches is"
        << volume << " cubic inches." << std::endl;
    return 0;
}
