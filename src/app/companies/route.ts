import { Manager } from "./manager"

export async function GET() {
    
    Manager()
    
    return Response.json({
        message: "hello"
    })
}