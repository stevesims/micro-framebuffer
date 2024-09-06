# Micro-framebuffer

A simple pixel framebuffer library primarily intended use with for embedded systems.

This is currently a work in progress.


## Features

This library provides a minimal framebuffer API with features to allow for the modification of individual pixels, or rows of pixels within that buffer.  Modification operations (will) include simple setting of pixels, and other bitwise operations such as AND, OR, XOR, and NOT to modify pixels.

Support for framebuffers that use a whole byte, or fractions of a byte, such as 1, 2, or 4 bits-per-pixel will be supported.  (Support for 3bpp may be included too, as may support for multi-byte pixels, such as 16bpp, 24bpp and 32bpp - the architecture is designed to allow for these, but it may not be a high priority to implement them.)

No assumptions are made about the display hardware, and the library is designed to be able to work with a wide range of hardware of varying capabilities.  No ability to display the framebuffer is included in this library.  It is intended to use this library in conjunction with a simple display driver to take the contents of the framebuffer and display it on a screen.

As the framebuffer provides only minimal operations to change the buffer contents, a higher-level library should be used to draw shapes, text, etc. on the framebuffer.  Palette management is also expected to be handled by the higher-level library.

With this split of functionality, the framebuffer and companion drawing libraries could be used on a variety of platforms.  The framebuffer and drawing libraries would be platform-independent, with only a relatively simple display driver required to be platform-specific.

Exact details of the features provided by this library are still being worked out, and will be documented here as they are finalised.


## Some history

Initially this library is being developed as a candidate to replace the underlying framebuffer functionality for the Agon Light VDP firmware.  The Agon Light is a modern retro computer system based on the eZ80 microcontroller and makes use of an ESP32-Pico-D4 microcontroller as it's video display processor (VDP).

Part of the motivation in developing this library is to provide a framebuffer that is not tied to the xtensa-based ESP32 platform.  As such it is intended that this library will be able to be used on a wide range of platforms, including other microcontroller-based systems, and potentially even on desktop systems too.

A limitation of the current Agon Light VDP firmware is that it is built on top of a library called `vdp-gl`, which is a fork of `fab-gl`.  The architecture of this library tightly couples its drawing code with its framebuffer and signal generation code.  This ties the codebase to the xtensa-based ESP32 platform.  Extending the library to support new functionality is difficult, requiring a lot of duplication, and the code is essentially not portable to other platforms.

An example of desired functionality in the Agon VDP would be the ability to redirect drawing operations into an off-screen framebuffer, and then blit that framebuffer to the display framebuffer.  This is very hard to achieve within the current architecture, making it impractical to implement.

We would also like to explore the possibility of using a different microcontroller to run the agon-vdp firmware, potentially for a future Agon-platform computer, and this library is part of that exploration.  A logical target for this exploration would be the new RP2350 microcontroller, and also newer ESP32 chips based on the RISC-V architecture.

It is therefore considered that a new framebuffer library should be developed which is more flexible and can be used on a wider range of platforms.  Other micro hardware projects are being developed that could make use of a simple and minimalistic framebuffer.  This library is the result of that effort.

A companion library will be developed to provide higher-level drawing functions on top of this library.  This library will aim to provide drawing functionality comparable to Apple's QuickDraw as provided by the original Mac OS, or the VDU command set provided by Acorn's RISC OS.



