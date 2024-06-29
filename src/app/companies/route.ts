export async function GET() {
    return Response.json([
        {id:0, name:'BUD', websites:['https://karrier.bud.hu/search/']},
        {id:1, name:'Praktiker', websites:['https://karrier.praktiker.hu/allasaink?category_id=&shop_id=332']},
        {id:2, name: 'KSH', websites:['https://www.ksh.hu/karrier']}
    ])
}