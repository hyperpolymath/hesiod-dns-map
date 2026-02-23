# SPDX-License-Identifier: PMPL-1.0-or-later
# Hesiod DNS configuration schema - Nickel type contracts

let ServiceEntry = {
  name | String,
  host | String,
  port | Number,
  protocol | String | default = "tcp",
}
in

let UserEntry = {
  username | String,
  uid | Number,
  gid | Number,
  gecos | String | default = "",
  home | String,
  shell | String | default = "/bin/bash",
}
in

let GroupEntry = {
  name | String,
  gid | Number,
  members | Array String | default = [],
}
in

let HesiodConfig = {
  domain | String,
  lhs | String,
  rhs | String,
  ttl | Number | default = 300,
  dns_port | Number | default = 53,
  http_port | Number | default = 8080,
  services | Array ServiceEntry | default = [],
  users | Array UserEntry | default = [],
  groups | Array GroupEntry | default = [],
}
in

{
  ServiceEntry = ServiceEntry,
  UserEntry = UserEntry,
  GroupEntry = GroupEntry,
  HesiodConfig = HesiodConfig,
}
