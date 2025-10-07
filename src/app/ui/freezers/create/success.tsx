"use client"
import {useRouter} from "next/navigation";
import {dummyFreezers} from "@/assets/dummy-data";
import {FaCircleCheck} from "react-icons/fa6";
import {useEffect} from "react";

export default function AddFreezerSuccess({id}: {id: number}) {
  const router = useRouter();
  console.log(id)
  console.log(dummyFreezers)
  console.log(dummyFreezers.filter(f => f.freezerId === Number(id)))
  const freezers = dummyFreezers.filter(f => f.freezerId === Number(id))
  console.log(freezers)
  if (!freezers || freezers.length === 0) {
    // todo: provide better error handling here.
    Error("Error - to be handled properly")
  }

  // useEffect(() => {
  //   setTimeout(() => router.push("/drawers/edit/10"), 5000)
  // })

  return (
    <div className="flex flex-col text-center justify-center items-center">
      <h1 className="text-green-500 text-5xl animate-pulse"><FaCircleCheck /></h1>
      <h2 className="text-2xl mt-4">Successfully created freezer {"'"}{freezers[0].name}{"'"}</h2>
      <p className="my-4">Let{"'"}s add some drawers next...</p>
    </div>
  )
}
