import { CardContainer, CardHeader, CardContent} from "@/components/Card";

export default function ProductCard({product}: {product: Api.Product}) {
  return (
    <CardContainer>
      <CardHeader title={product.name} editLink="/"/>
      <CardContent>
        <p>Expires after {product.expirationMonths} months</p>
      </CardContent>
    </CardContainer>
  )
}
