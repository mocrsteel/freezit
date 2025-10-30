"use client"
import {useRouter} from "next/navigation";
import {dummyFreezers} from "@/assets/dummy-data";
import {FaCircleCheck, FaCircleNotch} from "react-icons/fa6";
import {useEffect} from "react";
import Link from "next/link";

export default function AddFreezerSuccess({id}: {id: number}) {
  const REDIRECT_DELAY = 1000; // milliseconds
  const router = useRouter();
  const freezers = dummyFreezers.filter(f => f.freezerId === Number(id))
  if (!freezers || freezers.length === 0) {
    // todo: provide better error handling here.
    Error("Error - to be handled properly")
  }

  useEffect(() => {
    setTimeout(() => router.replace("/freezers/3"), REDIRECT_DELAY);
  })

  return (
    <div className="flex flex-col text-center justify-center items-center">
      <h1 className="text-green-500 text-5xl animate-pulse"><FaCircleCheck /></h1>
      <h2 className="text-xl mt-4">Created freezer</h2>
      <p className="text-2xl font-bold my-2">{freezers[0].name}</p>
      <p className="my-4">Let{"'"}s add some drawers next...</p>
      <div className="flex flex-row h-fit justify-center mt-4 items-center">
        <FaCircleNotch className="animate-spin mr-2" />
        <p className="h-full">Redirecting...</p>
      </div>
      <p className="text-xs mt-1 text-gray-300">Click <Link className="underline" href={"/freezers/3"}>here</Link> it you are stuck</p>
    </div>
  )
}
