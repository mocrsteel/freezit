import AddFreezerSuccess from "@/ui/freezers/create/success";

export default async function Page({params}: { params: Promise<{ id: number }> }) {
  const {id} = await params;
  return (
    <>
      <AddFreezerSuccess id={id} />
    </>
  )
}
