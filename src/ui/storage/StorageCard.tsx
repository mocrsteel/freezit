import { CardContainer, CardHeader, CardContent } from "@/components/Card";

export default function StorageCard({storage}: { storage: Api.StorageResponse }) {
  return (
    <CardContainer>
      <CardHeader title={storage.productName} editLink="/"/>
      <CardContent>
        <div is="product-info" className="w-1/2">
          <p><b>Weight:</b> {storage.weightGrams} g</p>
          <p><b>Expires:</b> {storage.expirationDate.toLocaleDateString()}</p>
        </div>
        <div is="location-info" className="w-1/2">
          <p><b>Freezer:</b> {storage.freezerName}</p>
          <p><b>Drawer:</b> {storage.drawerName}</p>
        </div>
      </CardContent>
    </CardContainer>
  )
}
