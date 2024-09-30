
export default function NewCardButton() {
  return (
    <div className="flex-1">
        <button className="btn btn-ghost text-l btn-outline" onClick={()=>(document.getElementById('new-card') as HTMLFormElement).showModal()}>
        Add new card
        </button>
        <dialog id="new-card" className="modal">
          <div className="modal-box p-4">
            <h3 className="font-bold text-lg">Add new card</h3>
            <div className="divider"></div>
            <form method="dialog">
              
              <div className="space-y-2.5">
                <input type="text" placeholder="Card ID" name="card-id" id="card-id" className="input input-bordered w-full max-w-xs" />
                <select name="card-set" id="card-set" className="select select-bordered w-full max-w-xs">
                  <option disabled selected>Card set</option>
                </select>
                <input type="number" min="1" placeholder="Amount of cards" name="card-amount" id="card-amount" className="input input-bordered w-full max-w-xs" />
                
              </div>
              <div className="modal-action">
                <button className="btn ">close</button>
                <button type="submit" className="btn btn-outline">Confirm</button>
              </div>
            </form>
          </div>
        </dialog>
    </div>
    );
  }