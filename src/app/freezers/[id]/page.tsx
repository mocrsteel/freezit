import EditFreezer from "@/ui/freezers/edit";

export default async function FreezerDrawersPage({params}: { params: Promise<{ id: string }> }) {
  const {id} = await params;
  return <EditFreezer id={id}/>
}
