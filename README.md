Obsidian Extract
================

Extract a note from your [Obsidian](https://obsidian.md/) vault, transform wiki-links and then use [Pandoc](https://pandoc.org) to produce a nice stand-alone HTML for sharing.

Prerequisites
-------------

- A recent version of [Pandoc](https://pandoc.org) on your `PATH`.

Installation
------------

`obsidian-extract` is just a single, stand-alone binary. [Grab the download for your platform of choice here](https://github.com/smutch/obsidian-extract/releases). 

Usage
-----

1. Set the location of your vault as an environment variable called `OBSIDIAN_VAULT` in your terminal:

    ```sh
    export OBSIDIAN_VAULT=/path/to/your/vault
    ```

2. While viewing the note you want to export in Obsidian, hit `CMD-P` to bring up the command palette and select `Copy file path`.

3. Back in your terminal, run:

    ```sh
    obsidian-extract '<PATH YOU COPIED IN STEP 2>'
    ```

    where you replace `<NOTE PATH>` appropriately. **Note that the single quotes must remain though!**

4. You will now have a new file called `note.html`. 🎉


For all available options and invocation methods see `obsidian-extract --help`.

To simply replace the wiki-links with standard markdown links, without calling Pandoc, use:

```sh
obsidian-extract --stdout ...
```

This is useful if you want to call Pandoc yourself and customise the output etc. e.g.

```sh
obsidian-extract --stdout 'path/to/my/note.md' | pandoc -f markdown --standalone --embed-resources -o note.html
```
