import { dummyFreezerResponse, dummyStorage } from "@/assets/dummy-data"


/**
 * Gets a single DisplayFreezer from the backend, based on its freezerId
 * @param id
 * @returns a single DisplayFreezer
 */
export const getDisplayFreezer = (id: number): DisplayFreezer => {
    return dummyFreezerResponse.filter(freezer => freezer.freezerId === id)[0]
}

export const getStorage = (freezerId?: number, drawerId?: number): Api.StorageResponse[] => {
    if (!freezerId && drawerId) {
        throw new Error("drawer is underspecified. Combination of drawerId + freezerId is required.")
    }
    if (!freezerId && !drawerId) {
        throw new Error("Function 'getStorage' requires either a freezerId or a drawerId.")
    }
    if (freezerId && drawerId) {
        const freezer = getDisplayFreezer(freezerId)
        const drawer = freezer.drawers.filter(drawer => drawer.drawerId === drawerId)[0]
        return dummyStorage.filter(storage => (storage.freezerName === freezer.name) && (storage.drawerName === drawer.name))
    } else  if (freezerId) {
        const freezer = getDisplayFreezer(freezerId)
        return dummyStorage.filter(storage => storage.freezerName === freezer.name)
    } else {
        throw new Error("Unknown error in `getStorage`.")
    }
}
