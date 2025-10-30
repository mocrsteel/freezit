export default function FreezerLayout({children, modal}: {children: React.ReactNode, modal: React.ReactNode}) {
  return (
    <>
      {modal}
      {children}
    </>
  )
}
