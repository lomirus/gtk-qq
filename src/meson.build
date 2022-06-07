conf = configuration_data()
conf.set_quoted('VERSION', meson.project_version())
conf.set_quoted('APPLICATION_ID', application_id)

config_output_file = configure_file(
  input: 'config.rs.in',
  output: 'config.rs',
  configuration: conf
)
# Copy the config.rs output to the source directory.
if build_machine.system() == 'windows'
  run_command(
    'Xcopy',
    config_output_file,
    meson.current_source_dir(),
    # overwrite exist file
    '/y'
    check: true
  )
else
  run_command(
    'cp',
    config_output_file,
    meson.current_source_dir(),
    check: true
  )
endif