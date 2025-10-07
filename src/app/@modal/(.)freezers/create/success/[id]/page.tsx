import Modal from "@components/modal";
import AddFreezerSuccess from "@/ui/freezers/create/success";

export default async function Page({params}: {params: Promise<{id: number}>}) {
  const {id} = await params;
  return (
    <Modal>
      <AddFreezerSuccess id={id} />
    </Modal>
  )
}
