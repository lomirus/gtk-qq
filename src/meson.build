global_conf = configuration_data()
global_conf.set_quoted('PKGDATA_DIR', pkgdatadir)

config_output_file = configure_file(
  input: 'config.rs.in',
  output: 'config.rs',
  configuration: global_conf
)
# Copy the config.rs output to the source directory.
run_command(
  'cp',
  config_output_file,
  meson.current_source_dir(),
  check: true
)
