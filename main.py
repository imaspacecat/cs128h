import cv2
import os
import threading
from queue import Queue

def save_frame(queue, output_dir):
    while True:
        frame_data = queue.get()
        if frame_data is None:
            break
        frame, count = frame_data
        frame_filename = os.path.join(output_dir, f"frame_{count:04d}.jpg")
        cv2.imwrite(frame_filename, frame)
        print(f"Saved frame {count} to {frame_filename}")
        queue.task_done()

def extract_frames(video_path, output_dir):
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    video = cv2.VideoCapture(video_path)
    success, frame = video.read()
    count = 0

    queue = Queue(maxsize=10)
    thread = threading.Thread(target=save_frame, args=(queue, output_dir), daemon=True)
    thread.start()

    while success:
        queue.put((frame, count))
        success, frame = video.read()
        count += 1

    queue.put(None)
    queue.join()
    video.release()
    print(f"Extraction complete. Total frames saved: {count}")

if __name__ == "__main__":
    video_path = "/home/spacecat/code/shrek-apple/media/shrek.mp4"
    output_dir = "/home/spacecat/code/shrek-apple/shrek-frames"
    extract_frames(video_path, output_dir)
