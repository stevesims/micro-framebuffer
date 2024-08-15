# Micro-framebuffer

A simple pixel framebuffer library primarily intended use with for embedded systems.

This is currently a work in progress.


## Features

Provides a minimal framebuffer API with features to allow for the modification of individual pixels, or rows of pixels within that buffer.  Modification operations (will) include simple setting of pixels, and other bitwise operations such as AND, OR, XOR, and NOT to modify pixels.

Support for framebuffers that use fractions of a byte, such as 1, 2, or 4 bits-per-pixel will be supported.  (Support for 3bpp may be included too.)

No assumptions are made about the display hardware, and the library is designed to be able to work with a wide range of display hardware of varying capabilities.  No ability to display the framebuffer is included in this library.

It is intended for this library to be used in conjunction with a simple display driver to take the contents of the framebuffer and display it on a screen.  Similarly a higher-level libary should be used to draw shapes, text, etc. on the framebuffer.  Palette management is expected to be handled by the higher-level library.

With this split of functionality, the framebuffer and companion drawing libraries could be used on a variety of platforms.  A minimal display driver, providing platform-specific code, would be required to display the framebuffer on a screen.  The framebuffer and drawing libraries would be platform-independent.

Exact details of the features provided by this library are still being worked out, and will be documented here as they are finalised.


## Some history

This library is being developed, along with companion libraries, as a potential replacement for the framebuffer functionality used in on the Agon Light VDP firmware.  The existing framebuffer support in the Agon Light firmware makes use of a library called `vdp-gl` which is a fork of `fab-gl`.  The architecture of this library tightly couples its drawing code with its framebuffer and signal generation code.  This ties the codebase to the xtensa-based ESP32 platform.  Extending the library to support new functionality is difficult, requiring a lot of duplication, and the code is essentially not portable to other platforms.

An example of desired functionality in the Agon VDP would be the ability to redirect drawing operations into an off-screen framebuffer, and then blit that framebuffer to the display framebuffer.  This is not possible within the current architecture, and not practical to implement.

It is therefore considered that a new framebuffer library should be developed which is more flexible and can be used on a wider range of platforms.  Other micro hardware projects are being developed that could make use of a simple and minimalistic framebuffer.  This library is the result of that effort.

A companion library will be developed to provide higher-level drawing functions on top of this library.  This library will aim to provide drawing functionality comparable to Apple's QuickDraw as provided by the original Mac OS, or the VDU command set provided by Acorn's RISC OS.



