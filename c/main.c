#include <stdio.h>

int main() {
    const double PI = 3.14159;
    double radius = 0;
    printf("Enter the radius of the sphere\n");
    scanf("%lf", &radius);
    double volume = PI * radius * radius * radius / 3;
    printf("The volume of the sphere with the radius of %lf inches is %lf cubic inches.", radius, volume);
    return 0;
}
