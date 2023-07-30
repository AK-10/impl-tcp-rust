# !/bin/bash

# reference: https://techblog.ap-com.co.jp/entry/2019/06/28/100439

set -eux

# add network namespace as host and router
sudo ip netns add host1
sudo ip netns add router
sudo ip netns add host2

# add network interface
sudo ip link add name host1-veth1 type veth peer name router-veth1
sudo ip link add name router-veth2 type veth peer name host2-veth1

# bind network interface to host and router
sudo ip link set host1-veth1 netns host1
sudo ip link set router-veth1 netns router
sudo ip link set router-veth2 netns router
sudo ip link set host2-veth1 netns host2

# add ip address to network interface
sudo ip netns exec host1 ip addr add 10.0.0.1/24 dev host1-veth1
sudo ip netns exec router ip addr add 10.0.0.254/24 dev router-veth1
sudo ip netns exec router ip addr add 10.0.1.254/24 dev router-veth2
sudo ip netns exec host2 ip addr add 10.0.1.1/24 dev host2-veth1

# enable network interface
sudo ip netns exec host1 ip link set host1-veth1 up
sudo ip netns exec router ip link set router-veth1 up
sudo ip netns exec router ip link set router-veth2 up
sudo ip netns exec host2 ip link set host2-veth1 up
sudo ip netns exec host1 ip link set lo up
sudo ip netns exec router ip link set lo up
sudo ip netns exec host2 ip link set lo up

# add routing
sudo ip netns exec host1 ip route add 0.0.0.0/0 via 10.0.0.254
sudo ip netns exec host2 ip route add 0.0.0.0/0 via 10.0.1.254

# enable ip forward on router
sudo ip netns exec router sysctl -w net.ipv4.ip_forward=1

# drop RST
# 今回の実装ではRSTセグメントを無視する
# Linuxがカーネルが持つプロトコルスタックと生ソケットの両方にパケットを渡す
# toytcpが流したSYNセグメントは宛先で動くOSのTCPとtoytcpの両方が受け取ることになる
# OSのTCPからすれば開けていないポートに対してSYNセグメントが到達したことになり、OS
# のTCPは送信元へRSTセグメントを送信し、コネクションの確立を拒否しようとする

# 正常時
# client --[SYN=1,ACK=0,SEQ=0,srcport=14000,destport=80]-> server
# client <-[SYN=1,ACK=1,SEQ=0,srcport=80,destport=14000]-- server
# client --[SYN=0,ACK=1,SEQ=1,srcport=14000,destport=80]-> server

# 異常時
# client --[SYN=1,ACK=0,SEQ=0,srcport=14000,destport=80]-> server
# client <-[SYN=1,ACK=1,SEQ=0,srcport=80,destport=14000]-- server(toytcp)
# client <-[SYN=1,ACK=1,SEQ=0,RST=1,srcport=80,destport=14000]-- server(OS) // OS側で80portは開けていないため、RSTパケットを返す
#
# RSTパケット:
# TCPのRSTフラグを1にしたもの
# 接続要求を拒絶したり、確立された接続を一方的に切断する際に送られる
sudo ip netns exec host1 sudo iptables -A OUTPUT -p tcp --tcp-flags RST RST -j DROP
sudo ip netns exec host2 sudo iptables -A OUTPUT -p tcp --tcp-flags RST RST -j DROP

# turn off checksum offloading
# セグメント破損の検出のため、チェックサムフィールドが存在する.
# 本来ＴＣＰ実装によって計算され埋められるが, CPUリソースの節約のため、NICにオフロードされる
# プロトコル実装の際に面倒なことになるのでオフにする
sudo ip netns exec host2 sudo ethtool -K host2-veth1 tx off
sudo ip netns exec host1 sudo ethtool -K host1-veth1 tx off
