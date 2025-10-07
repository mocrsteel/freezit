import Modal from "@components/modal"
import AddFreezerFailed from "@/ui/freezers/create/failure";

export default async function Page({params}: {params: Promise<{name: string}>}) {
  const {name} = await params
  return (
    <Modal>
      <AddFreezerFailed name={name} />
    </Modal>
  )
}
