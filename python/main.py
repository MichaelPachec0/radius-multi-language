#!/usr/bin/env python3


def main():
    PI = 3.14159
    try:
        radius = float(input("Enter the radius of the sphere: "))
    except ValueError as e:
        print(e)
        return
    volume = PI * radius ** 3 / 3
    print(f"The volume of a sphere with a radius of {radius} inches is {volume} cubic inches.")


if __name__ == '__main__':
    main()

