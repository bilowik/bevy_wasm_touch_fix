A plugin to fix the issue with touch inputs in web-embedded Bevy applications where touch 
input positions are relative to the web page's viewport and not the canvas. Fixes both UI elements and when 
reading TouchInput events or the Touches resource. 

Add the plugin and ensure the canvas id in the html where the bevy app is rendered matches 
the one set in the WindowPlugin and in the PrimaryCanvasId resource. The default for this 
value in the resource is `main-canvas`

For example, in the html where the bevy app is rendered, specify the id of the canvas:
```html
<canvas id="main-canvas"></canvas>
```

Then you need to configure the Window plugin to render to that ID as well:
```rust
App::new()
    .with_plugins(DefaultPlugins.set(WindowPlugin {
        primary_Window: Some(Window {
            canvas: Some(String::from("#main-canvas")),
            ..default()
        }),
        ..default()
    }))
    // ...
```

and if you utilize a different ID than the default `main-canvas`, you will need to change the
PrimaryCanvasId resource.
```rust
        //...
        .insert_resource(PrimaryCanvasId(String::from("my-custom-id")));
        //...
```


## NOTE
My testing of this has been limited to very basic web pages. I'm pretty certain this isn't the case, 
but is possible that the `web-sys::Element::get_bounding_client_rect` alone may not be all that is 
needed to calculate an accurate offset for complex web pages. 

In the case that additional adjustments are needed on your end, you should utilize the 
[AdditionalTouchOffset] resource and add any additional offset there that is needed and it 
will be included in the offsetting of the touch events.

I also do not know the exact implications (if any) of pulling off the events and pushing them back on. 
I guaranteed that both a and b queues remain the same (+ the offset of course) before and after the offsetting,
but the event count will increment twice as fast. 
