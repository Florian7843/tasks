cargo = find_program('cargo', required: true)

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: 'release',
  console: true,
  command: [
    cargo, 'build'
  ]
)