/**
 * Compile it with
   clang++ glfw_basic.cpp -I../output -lglfw -o glfw_basic.out
 **/

#include <cstddef>
#include <iostream>
#include <dlfcn.h>

#define GLFW_INCLUDE_NONE
#include <GLFW/glfw3.h>

#include <kaxel/gl.h>

typedef void (*_ty_glClear)(GLbitfield mask);
_ty_glClear glClear = nullptr;

typedef void (*_ty_glClearColor)(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha);
_ty_glClearColor glClearColor = nullptr;

void LoadOpenGLFunctions() {
    glClear = (_ty_glClear)glfwGetProcAddress("glClear");
    glClearColor = (_ty_glClearColor)glfwGetProcAddress("glClearColor");
}

int main(void)
{
    GLFWwindow* window;

    /* Initialize the library */
    if (!glfwInit()) return -1;

    /* Create a windowed mode window and its OpenGL context */
    window = glfwCreateWindow(640, 480, "Hello World", NULL, NULL);
    if (!window)
    {
        glfwTerminate();
        return -1;
    }

    /* Make the window's context current */
    glfwMakeContextCurrent(window);

    LoadOpenGLFunctions();

    glClearColor(0.92156, 0.25098, 0.20392, 1.0);

    /* Loop until the user closes the window */
    while (!glfwWindowShouldClose(window))
    {
        /* Render here */
        glClear(GL_COLOR_BUFFER_BIT);

        /* Swap front and back buffers */
        glfwSwapBuffers(window);

        /* Poll for and process events */
        glfwPollEvents();
    }

    glfwTerminate();
    return 0;
}
