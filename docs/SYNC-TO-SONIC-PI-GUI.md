## Syncing sonic-pi-tool to a Runningi instance of Sonic PI GUI


## Why is this needed?

From version 4.0 and on, Sonic Pi's internal processes assing ports to communicate with one another dynamically (as opposed to using a pre-established one).
Additionally, they added the concept of a token which is required for messages to be processed by the audio server.

## How to sync?
If you want to use sonic-pi-tool with a running instance of Sonic Pi, you need the port of the audio server, and the token to make sure your messages are treated as valid.


### From Linux or Mac OS

From terminal run the following command:
```sh
> ps x | grep spider
```

This allows us to find the spider-server, which is the one that responds to the OSC commands sonic-pi-tool will send.
You should see something like:
```sh
> ps x | grep spider
52800   /Applications/Sonic Pi.app/Contents/Resources/app/server/native/ruby/bin/ruby
      --enable-frozen-string-literal -E utf-8
      /Applications/Sonic Pi.app/Contents/Resources/app/server/ruby/bin/spider-server.rb
      -u 33926 33927 33928 33928 4560 33929 33933 1839294199
```

What we care about here are 2 numbers - The port of the server (the first number, right after the `-u` - `33926` in our example above) and the last number, which is the token (`1839294199` in the example above)

With those two number we can call `sonic-pi-tool sync` and it will take care of setting up the configuration required:
```sh
> sonic-pi-tool sync 33926 1839294199
##  Now sonic-pi-tool is sync'd with that instance of Sonic Pi GUI

> sonic-pi-tool check
# => Sonic Pi server listening on port 33926
```

## From Windows
