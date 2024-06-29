import { CareerSite } from "@/model/careerSite";

export default function List({careerSites} : {careerSites:CareerSite[]}) {
    const listItems = careerSites.map (careerSite => 
        <li key={careerSite.id}>
            {careerSite.name}
        </li>
    )
    
    return (
        <ol>{listItems}</ol>
    )
}