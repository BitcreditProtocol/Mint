import {ComponentPreview, Previews} from '@react-buddy/ide-toolbox'
import {PaletteTree} from './palette'
import EndorsePage from "../components/pages/MintPage";

const ComponentPreviews = () => {
    return (
        <Previews palette={<PaletteTree/>}>
            <ComponentPreview path="/EndorsePage">
                <EndorsePage/>
            </ComponentPreview>
        </Previews>
    )
}

export default ComponentPreviews