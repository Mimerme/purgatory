# GLSL Uniform Parameters

## Waterwheel is basically Shadertoy++
Here are the direct 1-to-1 mappings for their uniform input variables. For help writing GLSL shaders check out Shadertoy's [tutorial](https://www.shadertoy.com/howto). Note that Waterwheel only support image shaders as of now

## Getting started
Copy the template file and start with one of the entry points
```void mainImage( out vec4 fragColor, in vec2 fragCoord );```

## Differences
Shadertoy primarily works as 


| **Data Type**   | **Shadertoy Name** | **Waterwheel Name** | **Documentation**
| ----------- | ----------- | ----------- | ------------|
| Vec4        | iMouse     | mouse         | Vec4(mouse_x, mouse_y, left_down, right_down)
| float       | iTime      | time          | playback time (in seconds)
| float       | iTimeDelta | time_delta    | playback time (in seconds)
| int         | iFrame     | current_frame | shader playback frame
| Vec4        | iDate      | current_frame | Vec4(year, month, day, seconds) 
| Vec2        | iResolution | resolution | Vec2(window_width, window_height) (in pixels)

