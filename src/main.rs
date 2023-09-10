use windows::{
    core::*,
    Graphics::Capture::Direct3D11CaptureFramePool,
    Graphics::Capture::GraphicsCapturePicker,
    Graphics::Imaging::BitmapDecoder,
    Media::Ocr::OcrEngine,
    Storage::{FileAccessMode, StorageFile},
};

fn main() -> Result<()> {
    futures::executor::block_on(decode_ocr())
}

async fn decode_ocr() -> Result<()> {
    let mut message = std::env::current_dir().unwrap();
    message.push("message.png");

    let file =
        StorageFile::GetFileFromPathAsync(&HSTRING::from(message.to_str().unwrap()))?.await?;
    let stream = file.OpenAsync(FileAccessMode::Read)?.await?;

    let decode = BitmapDecoder::CreateAsync(&stream)?.await?;
    let bitmap = decode.GetSoftwareBitmapAsync()?.await?;

    let engine = OcrEngine::TryCreateFromUserProfileLanguages()?;
    let result = engine.RecognizeAsync(&bitmap)?.await?;

    println!("{}", result.Text()?);
    Ok(())
}

async fn _capture_frame() -> Result<()> {
    let picker = GraphicsCapturePicker::new()?;
    let _ = picker.PickSingleItemAsync().unwrap().GetResults()?;
    Ok(())
}
