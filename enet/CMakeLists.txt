cmake_minimum_required(VERSION 2.6)
project(enet)

add_library(enet 
callbacks.c 
host.c 
list.c 
packet.c 
peer.c 
protocol.c 
unix.c 
win32.c 

callbacks.h 
enet.h 
list.h 
protocol.h 
time.h 
types.h 
unix.h 
utility.h 
win32.h)
if(WIN32)
  target_link_libraries(enet ws2_32 winmm)
endif()
