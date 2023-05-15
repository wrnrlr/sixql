# SIXQL

Experimental extension of the Postgres database that provides inline plotting functionality.
It can display high quality graphics right in the same terminal used to write SQL.

## Requirements
* [libsixel](https://github.com/saitoha/libsixel#readme):
    * Ubuntu: `sudo apt install libsixel-dev`
    * OSX: `brew install libsixel` (is broken ATM)
    * NIX: TODO
* [pgrq](https://github.com/tcdi/pgrx#readme): `cargo install --locked cargo-pgrx; cargo pgrx init`

Restart your terminal and run the following command to verify if SIXEL is supported,
it should display a small red rectangle: `cat sample.sixel`.

## API (WIP)

The system is currently a bit of a hack in that we log the images to stdout of `psql` instead of returning them as a result of a function. 
This is because the postgresql wire format will filter our the escape sequences needed for SIXEL to work, but that is probably a good thing.

Test if everything is working in `psql`.

`select hello_sixql();`

You can also suppress the empty result output with something like `select hello_sixql() \gexec`
or even stranger: `do $$ begin perform hello_sixql(); end; $$;`.
Please let me know if you have a better idea.

There should probably be one main function that rasterizer a declarative description of the image ala. [The Language of Graphics](https://www.cs.uic.edu/~wilkinson/Publications/gpl.pdf) or something like [this](https://github.com/ajstarks/dubois-data-portraits/blob/master/toc.pdf) implementation of W.E.B Dubois's Data Portraits.
