module.exports = {
  contract: {
    permission: '0xffffffffffffffffffffffffffffffffff021010',
    authorization: '0xffffffffffffffffffffffffffffffffff020006',
    permissionManagement: '0xffffffffffffffffffffffffffffffffff020004',
    roleManagement: '0xffffffffffffffffffffffffffffffffff020007',
    groupManagement: '0xffffffffffffffffffffffffffffffffff02000a',
    group: '0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009',
    quota: '0xffffffffffffffffffffffffffffffffff020003',
    nodeManager: '0xffffffffffffffffffffffffffffffffff020001',
    chainManager: '0xffffffffffffffffffffffffffffffffff020002',
    admin: '0xffffffffffffffffffffffffffffffffff02000c',
    roleAuth: '0xffffffffffffffffffffffffffffffffff02000d',
    autoExecAddr: '0xffffffffffffffffffffffffffffffffff020013',
    versionManager: '0xffffffffffffffffffffffffffffffffff020011',
  },
  localServer: 'http://127.0.0.1:1337',
  remoteServer: 'http://xx.xx.xx.xx:1337',
  // TODO delete. use the real exist contract
  testAddr: ['0x1a702A25c6bcA72B67987968f0BFb3A3213c5600', '0x1a702A25c6BCA72b67987968f0BFb3a3213c5601', '0x1A702a25C6bCa72b67987968f0bfb3A3213C5602'],
  testFunc: ['0xf036ed56', '0x3482e0c9', '0xf036ed56'],
  testBin: '6060604052341561000f57600080fd5b60d38061001d6000396000f3006060604052600436106049576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b114604e5780636d4ce63c14606e575b600080fd5b3415605857600080fd5b606c60048080359060200190919050506094565b005b3415607857600080fd5b607e609e565b6040518082815260200191505060405180910390f35b8060008190555050565b600080549050905600a165627a7a7230582020642d4bfc8bb29cfd3390d1aafea86ec7219a70889a640325d7fabb0b0534960029',
  testSender: {
    address: '0x9dcd6B234E2772C5451Fd4ccf7582f4283140697',
    privkey: '993ef0853d7bf1f4c2977457b50ea6b5f8bc2fd829e3ca3e19f6081ddabb07e9',
  },
  permissions:
    [
      '0xfFfFffFffffFFfffFfFfFffFFFfFFfFFFf021010',
      '0xFFfFfffffFFffFfffFffffffFFfFfFfFfF021011',
      '0xfFFfFFfFFFFffffFFFFFfffffFFFFFFFFf021012',
      '0xfFFFffFffFfffFffFfffFfFFfFFFfFffFf021013',
      '0xfFFFffFfffFFFFffFfFffffFfFFFfffFfF021014',
      '0xFFFFFfffffFFFfFfffffFfFfffffFFffFf021015',
      '0xfFfFFFFFffFFfFFfFFfFFfFfFFfffFFffF021016',
      '0xFFFFffFFFFfFFFFFFfFFffffFFFFFFFFff021017',
      '0xfFFFfFfFFFFFFffFfFFFFfffFffFfFFFFF021018',
      '0xfFFffffffFffFffFFFFFFFFFffFfffFFfF021019',
      '0xFFFFffffffffFFfFffFffFFfFfFfFffFFf02101A',
      '0xFFfFfffFffffffffFFfFfFFFFfFFfFfFFF02101B',
      '0xFFFfFFfffFFffFffffffFFFFFFfFFffffF02101c',
      '0xFFffFFFFfFFFFFFfffFfFFffFfFFFFfFFf021000',
      '0xffFFffffFfffFFFfffffFFfFFffFFfFFFf021001',
      '0xFfFfFFffFffffFffffffffFFFFffFfFFFF021020',
      '0xFffFFFfFfFFFfFfFfFfFFfffFFFFFffFFF021021',
      '0xFffFfffFFffFFFFfFFFFFfFfFFFfFFFfFF021022',
      '0xffFfffFfffFfFFFFFfFfFffFFfFfffFffF021023',
      '0xfffffFfFfFFfFFffFfFffFFFFfFFFfFffF021024',
      '0xFFFffFFFfFfFFffffffFfFfFFfFfffFFFf021025',
      '0xfFFfffFfFfffFffFFFfFfFFfFffFFfFFFf021026',
      '0xffFFffFFffFFfFFFFffffFfFFFFFFFffFf021027',
      '0xFffFffFffFfFFfFfffFffffffffFffFfFF021028',
    ],
  rootGroupPermissions: [
    '0xFFffFFFFfFFFFFFfffFfFFffFfFFFFfFFf021000',
    '0xffFFffffFfffFFFfffffFFfFFffFFfFFFf021001',
  ],
  resources: [
    ['0xffffffffffffffffffffffffffffffffff021000', '0x00000000'],
    ['0xffffffffffffffffffffffffffffffffff021001', '0x00000000'],
    ['0xffffffffffffffffffffffffffffffffff020004', '0xfc4a089c', '0x98a05bb1', '0xf036ed56', '0x6446ebd8', '0x537bf9a3', '0x0f5aa9f3', '0x52c5b4cc', '0x3482e0c9', '0xa5925b5b', '0xba00ab60'],
    ['0xffffffffffffffffffffffffffffffffff020007', '0x551ef860', '0x54b025c5', '0x0773e6ba', '0x17b2e350', '0xd9c090a0', '0xa32710eb', '0xa8319481', '0xc631e758'],
    ['0xffffffffffffffffffffffffffffffffff02000a', '0xd7cd7209', '0xbaeb8cad', '0x2c84e31f', '0xd86df333', '0x7eafcdb1'],
  ],
  superAdmin: {
    address: '0x4b5Ae4567aD5D9FB92Bc9aFd6A657e6fA13a2523',
    privkey: '5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6',
  },
  rootGroup: '0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009',
};
