export const dummyFreezers: Array<Api.Freezer> = [
  {
    freezerId: 1,
    name: "Garage",
  },
  {
    freezerId: 2,
    name: "Kelder",
  },
  {
    freezerId: 3,
    name: "Berging",
  },
  {
    freezerId: 4,
    name: "Gang",
  },
  {
    freezerId: 10,
    name: "Dummy created freezer",
  },
]

export const dummyDrawers: DrawerOverview[] = [
  {
    drawerId: 1,
    name: "Schuif 1",
    freezerId: 1,
    itemCount: 10
  },
  {
    drawerId: 2,
    name: "Schuif 2",
    freezerId: 1,
    itemCount: 10
  },
  {
    drawerId: 3,
    name: "Schuif 3",
    freezerId: 1,
    itemCount: 15
  },
  {
    drawerId: 4,
    name: "Bak",
    freezerId: 2,
    itemCount: 8
  },
  {
    drawerId: 5,
    name: "Schuif 1",
    freezerId: 3,
    itemCount: 3
  },
  {
    drawerId: 6,
    name: "Schuif 2",
    freezerId: 3,
    itemCount: 25
  },
  {
    drawerId: 7,
    name: "Schuif 3",
    freezerId: 3,
    itemCount: 18
  },
  {
    drawerId: 8,
    name: "Schuif 4",
    freezerId: 3,
    itemCount: 20
  },
]

export const dummyFreezerResponse: DisplayFreezer[] = dummyFreezers.map(freezer => {
  return {
    ...freezer,
    drawers: dummyDrawers.filter(drawer => drawer.freezerId === freezer.freezerId),
    totalItemCount: dummyDrawers.filter(drawer => drawer.freezerId === freezer.freezerId).reduce((total, drawer, _) => {
      return total + drawer.itemCount
    }, 0)
  }
})

export const dummyProducts: Api.Product[] = [
  {
    productId: 1,
    name: "Wortelen",
    expirationMonths: 12
  },
  {
    productId: 2,
    name: "Brocoli",
    expirationMonths: 12
  }
]

export const dummyStorage: Array<Api.StorageResponse> = [
  {
    storageId: 1,
    weightGrams: 500.1,
    inStorageSince: new Date("2023-12-03"),
    outStorageSince: undefined,
    productName: "Brocoli",
    drawerName: "Schuif 1",
    freezerName: "Garage",
    expiresInDays: 124,
    expirationDate: new Date("2024-07-04")
  },
  {
    storageId: 2,
    weightGrams: 490.1,
    inStorageSince: new Date(2023, 9, 12),
    outStorageSince: undefined,
    productName: "Brocoli",
    drawerName: "Schuif 1",
    freezerName: "Berging",
    expiresInDays: 424,
    expirationDate: new Date(2023, 5, 28)
  },
  {
    storageId: 3,
    weightGrams: 521.1,
    inStorageSince: new Date(2024, 1, 15),
    outStorageSince: undefined,
    productName: "Wortelen",
    drawerName: "Schuif 2",
    freezerName: "Garage",
    expiresInDays: 144,
    expirationDate: new Date(2026, 6, 15)
  },
  {
    storageId: 4,
    weightGrams: 321.1,
    inStorageSince: new Date(2024, 1, 15),
    outStorageSince: undefined,
    productName: "Wortelen",
    drawerName: "Schuif 1",
    freezerName: "Berging",
    expiresInDays: 24,
    expirationDate: new Date(2026, 2, 29)
  },
]

export const dummyStorageCount: StorageCount[] = [
  {
    name: "Spagettisaus",
    count: 5,
    storageIdList: [4, 5, 6, 7, 8],
  },
  {
    name: "Brocoli",
    count: 3,
    storageIdList: [1, 2, 3]
  },
  {
    name: "Groentensoep",
    count: 1,
    storageIdList: [9, 10, 11]
  }
]

export const dummyExpiresNext = [
  {
    storageId: 4,
    weightGrams: 321.1,
    inStorageSince: new Date(2024, 1, 15),
    outStorageSince: undefined,
    productName: "Wortelen",
    drawerName: "Schuif 1",
    freezerName: "Berging",
    expiresInDays: 24,
    expirationDate: new Date(2026, 2, 29)
  }, {
    storageId: 8,
    weightGrams: 255.1,
    inStorageSince: new Date(2025, 1, 15),
    outStorageSince: undefined,
    productName: "Wortelsoep",
    drawerName: "Schuif 2",
    freezerName: "Berging",
    expiresInDays: 10,
    expirationDate: new Date(2026, 2, 29)
  }, {
    storageId: 230,
    weightGrams: 535.1,
    inStorageSince: new Date(2025, 1, 15),
    outStorageSince: undefined,
    productName: "Gehakt",
    drawerName: "Schuif 4",
    freezerName: "Berging",
    expiresInDays: 1,
    expirationDate: new Date(2026, 2, 29)
  },
]
