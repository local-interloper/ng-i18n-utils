# ng-i18n-utils

_ng-i18n-utils_ is a set of utilities for use with the Angular web framework's `@angular/localize` package.
Angular themselves said that they will not be addressing the issue of localization file changes themselves, 
so I've decided to create a small utility that generates and updates different language i18n files automatically.

## How to install:

_ng-i18n-utils_ is a CLI program. You can compile it, you can download it if I made a release by the time
you are reading this. All of its dependencies are cross-platform so you should in theory be able to 
compile it on whichever system, but I can confirm that it works on Linux. Just make sure you shove the binary 
somewhere within your `$PATH` or `%PATH%` depending on your OS.

## How to use:

Generate your i18n files by running the following command. Please note that they must be in JSON format.
```shell
ng extract-i18n --output-path i18n --format json
```

This should create `messages.json` under the `i18n` directory. Then we can create or update our i18n
files by running the following command.
```shell
ng-i18n-utils update i18n/messages.json de it
```

The `de` and `it` arguments are here used as an example. The angular standard way of naming i18n files 
is using 2 letters before the ".json" suffix (e.g. `messages.de.json`) so those are generated from these 
arguments.

Running that last command should create or update `messages.de.json` and `messages.it.json`. In case they 
already existed before running the command they will be updated with the missing keys from the source i18n
file.