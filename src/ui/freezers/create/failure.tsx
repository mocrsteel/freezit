"use client"
import {useRouter} from "next/navigation";

export default function AddFreezerFailed({name}: {name: string}) {
  const router = useRouter();
  return (
    <div className="w-full p-6 h-fit text-center flex flex-col justify-center center-content">
      <h1 className="text-3xl w-full text-center pb-4 animate-pulse">❌</h1>
      <h2 className="text-xl font-bold w-full text-center">Freezer creation failed!</h2>
      <p className="text-sm my-8">Something went wrong and I could not create freezer {name} for you.</p>
      <button className="px-4 py-2 rounded-2xl text-white transition-colors hover:transition-colors hover:bg-apple-600/90 bg-apple-600" onClick={() => { router.back() }}>Go back</button>
    </div>
  )
}
