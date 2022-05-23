project(
  'gtk-qq',
  'rust',
  version : '0.1.0',
  default_options : ['warning_level=3'])

prefix = get_option('prefix')
datadir = prefix / get_option('datadir')
pkgdatadir = datadir / meson.project_name()

application_id = 'indi.lomirus.gtk-qq'

subdir('src')
subdir('assets')
