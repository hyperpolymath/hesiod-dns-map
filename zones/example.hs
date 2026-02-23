; SPDX-License-Identifier: PMPL-1.0-or-later
; Example Hesiod zone file for flatracoon.internal
; Records use HS (Hesiod) class with TXT resource type
;
; SOA record
@ IN SOA ns.flatracoon.internal. admin.flatracoon.internal. (
    2026020801 ; serial (YYYYMMDDNN)
    3600       ; refresh
    900        ; retry
    604800     ; expire
    300        ; minimum TTL
)

; NS record
@ IN NS ns.flatracoon.internal.

; --- Service records (HS class TXT) ---
; Format: <name>.service.ns.flatracoon.internal. HS TXT "<host>:<port>:<protocol>"

twingate.service.ns    300 HS TXT "twingate.svc:443:tcp"
zerotier-api.service.ns 300 HS TXT "zerotier.svc:9993:udp"
ipfs-gateway.service.ns 300 HS TXT "ipfs.svc:8080:tcp"
ipfs-api.service.ns    300 HS TXT "ipfs.svc:5001:tcp"
orchestrator.service.ns 300 HS TXT "orchestrator.svc:4000:tcp"
dashboard.service.ns   300 HS TXT "dashboard.svc:4001:tcp"

; --- Passwd records (HS class TXT) ---
; Format: <username>.passwd.ns.flatracoon.internal. HS TXT "<user>:*:<uid>:<gid>:<gecos>:<home>:<shell>"

admin.passwd.ns    300 HS TXT "admin:*:1000:1000:FlatRacoon Admin:/home/admin:/bin/bash"
operator.passwd.ns 300 HS TXT "operator:*:1001:1001:FlatRacoon Operator:/home/operator:/bin/bash"

; --- Group records (HS class TXT) ---
; Format: <group>.group.ns.flatracoon.internal. HS TXT "<group>:*:<gid>:<members>"

operators.group.ns  300 HS TXT "operators:*:1001:admin,operator"
netadmin.group.ns   300 HS TXT "netadmin:*:1002:admin"
monitoring.group.ns 300 HS TXT "monitoring:*:1003:admin,operator"

; --- Filsys records (HS class TXT) ---
; Format: <name>.filsys.ns.flatracoon.internal. HS TXT "<type> <path> <server>:<export> <mode>"

home.filsys.ns     300 HS TXT "nfs /home nfsserver:/export/home rw"
shared.filsys.ns   300 HS TXT "nfs /shared nfsserver:/export/shared ro"
