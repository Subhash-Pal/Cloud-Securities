# Root Folder IP-Blocker 

The root folder contains 6 folder/directory,

out of 6 folder 3 are carrying the code to execute the IP-blocker.

1)Blocker-API folder contain server/client respectively.

./IP-Blocker/Blocker-API/Server

./IP-Blocker/Blocker-API/client

2)Policy folder Contains (output.json file which is generated by grpc stream)

3)xdp-drop (ebpf code to block ip ,generated by grpc and stored in policy/output.json)